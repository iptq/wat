#!/bin/bash

gotestcover -v -race -coverprofile=coverage.out ./... && go tool cover -html=coverage.out
