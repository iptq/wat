// Package conf lets you manage configuration files in the easiest way possible, without the unnecessary pain.
package conf

import (
	"errors"
	"fmt"
	"io/ioutil"
	"os"
	"reflect"
	"strconv"
	"strings"
)

// The only custom errors this package will return.
var (
	ErrNoFile     = errors.New("conf: the configuration file doesn't exist")
	ErrNotAStruct = errors.New("conf: the passed into/from variable is not a pointer to a struct")
)

// Load unmarshals a file into the struct passed as the argument "into".
func Load(into interface{}, filename string) error {
	intoValue := reflect.ValueOf(into)
	if intoValue.Kind() != reflect.Ptr || intoValue.Elem().Kind() != reflect.Struct {
		return ErrNotAStruct
	}
	intoValue = intoValue.Elem()
	f, err := ioutil.ReadFile(filename)
	if os.IsNotExist(err) {
		return ErrNoFile
	}
	if err != nil {
		return err
	}

	return loadRaw(intoValue, f)
}

// LoadRaw allows you to load into a struct some raw data bytes.
func LoadRaw(into interface{}, data []byte) error {
	intoValue := reflect.ValueOf(into)
	if intoValue.Kind() != reflect.Ptr || intoValue.Elem().Kind() != reflect.Struct {
		return ErrNotAStruct
	}
	intoValue = intoValue.Elem()
	return loadRaw(intoValue, data)
}

func loadRaw(intoValue reflect.Value, data []byte) error {
	fvs := Parse(data)
	for _, v := range fvs {
		for i := 0; i < intoValue.Type().NumField(); i++ {
			field := intoValue.Type().Field(i)
			if !intoValue.Field(i).CanSet() {
				continue
			}
			if field.Name == v.Field {
				switch field.Type.Kind() {
				case reflect.String:
					intoValue.Field(i).SetString(v.Value)
				case reflect.Bool:
					boolVal, err := strconv.ParseBool(v.Value)
					if err != nil {
						return err
					}
					intoValue.Field(i).SetBool(boolVal)
				case reflect.Int, reflect.Int8, reflect.Int16, reflect.Int32, reflect.Int64:
					intVal, err := strconv.ParseInt(v.Value, 10, 64)
					if err != nil {
						return err
					}
					intoValue.Field(i).SetInt(intVal)
				case reflect.Uint, reflect.Uint8, reflect.Uint16, reflect.Uint32, reflect.Uint64:
					uintVal, err := strconv.ParseUint(v.Value, 10, 64)
					if err != nil {
						return err
					}
					intoValue.Field(i).SetUint(uintVal)
				case reflect.Float32, reflect.Float64:
					floatVal, err := strconv.ParseFloat(v.Value, 64)
					if err != nil {
						return err
					}
					intoValue.Field(i).SetFloat(floatVal)
				}
			}
		}
	}

	return nil
}

// MustLoad has the same behaviour as Load, but panics if it returns an error.
func MustLoad(into interface{}, filename string) {
	if err := Load(into, filename); err != nil {
		panic(err)
	}
}

// MustLoadRaw has the same behaviour as LoadRaw, but panics if it returns an error.
func MustLoadRaw(into interface{}, data []byte) {
	if err := LoadRaw(into, data); err != nil {
		panic(err)
	}
}

// Export uses ExportRaw to put the data into a file, specified with its name.
func Export(from interface{}, filename string) error {
	data, err := ExportRaw(from)
	if err != nil {
		return err
	}
	return ioutil.WriteFile(filename, data, 0644)
}

// ExportRaw can create a []byte that can then be loaded back by LoadRaw to get a struct's original form back.
// I suck at explaining stuff.
func ExportRaw(from interface{}) ([]byte, error) {
	fromValue := reflect.ValueOf(from)
	if fromValue.Kind() == reflect.Ptr {
		return ExportRaw(fromValue.Elem().Interface())
	}
	if fromValue.Kind() != reflect.Struct {
		return []byte{}, ErrNotAStruct
	}
	return exportRaw(fromValue), nil
}

func exportRaw(fromValue reflect.Value) []byte {
	var ret []byte
	for i := 0; i < fromValue.Type().NumField(); i++ {
		curfield := fromValue.Field(i)
		curfieldType := fromValue.Type().Field(i)

		// Dirty hack to ignore that field if we don't support that type.
		if !((curfield.Kind() >= reflect.Bool && curfield.Kind() <= reflect.Uint64) ||
			curfield.Kind() == reflect.String || curfield.Kind() == reflect.Float32 ||
			curfield.Kind() == reflect.Float64) {
			continue
		}

		/* guten */ tag := curfieldType.Tag.Get("description")
		if tag != "" {
			tag = strings.Replace(tag, "\n", "\n; ", -1)
			ret = append(ret, []byte("; "+tag+"\n")...)
		}
		ret = append(ret, []byte(Escape(curfieldType.Name)+"="+Escape(fmt.Sprint(curfield.Interface()))+"\n")...)
	}
	return ret
}

// MustExport panics if Export returns an error, removing error checking from your code. For the lazy.
func MustExport(from interface{}, filename string) {
	if err := Export(from, filename); err != nil {
		panic(err)
	}
}

// MustExportRaw panics if ExportRaw returns an error, removing error checking from your code. For the lazy.
func MustExportRaw(from interface{}) []byte {
	data, err := ExportRaw(from)
	if err != nil {
		panic(err)
	}
	return data
}
