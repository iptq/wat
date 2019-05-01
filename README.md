WaT
===

Drop-in WakaTime replacement.

Contents
--------

- [Server-Side Installation](#server-side-installation)
- [Client-Side Installation](#client-side-installation)
- [Why?](#why)
- [Roadmap](#roadmap)
- [Contact](#contact)

Server-Side Installation
------------------------

First, you're going to need to set up the WaT server. The server is looking for a config file in the current directory named `wat.{yml,json,toml}`. Here's a full configuration with descriptions and default values, in YAML. If you choose to use a different format, you should take care to follow the syntax and conventions of that format.

```yml
# The address for the server to bind to, in the format host:port. Using hostnames is supported.
bind_address: ":6800"

# Database provider, can be either { postgres | mysql | sqlite3 }. All three are supported out of the box.
database_provider: "sqlite3"

# The database address, a DSN for either Postgres or MySQL, and a relative file address for Sqlite3.
database: "wat.db"

# Whether or not registration is open to anyone who views your page. If disabled, users will have to be manually registered through the CLI.
# Note: this is false by default
registration_enabled: false
```

Make sure the server is running from a location that your computer can reach, since API calls will be made to this server. If you're just serving to yourself, running it on localhost should do the trick. Otherwise, make sure at least the API is reachable.

Client-Side Installation
------------------------

Under your `~/.wakatime.cfg`, add a line indicating where your `api_url` is pointing to. For example,

    api_url=https://wat.mzhang.me/api/v1/heartbeats.bulk

Since most WakaTime editor plugins respect this configuration file, they should be pointed at the correct place as long as you set this value.

Why?
----

I've nothing against WakaTime, and I'm personally very grateful for the community they've built around tracking coding statistics. But at the end of the day, I'd like to keep my own data, and hopefully you do too. This project gives you the option to host a WakaTime-compatible server while allowing you to continue using the great plugins that people have developed for WakaTime.

Roadmap
-------

General usability:

- [] Web interface
  - [x] Registration
  - [ ] Login
  - [ ] Dashboard
    - [ ] Day-by-day breakdown
    - [ ] Project breakdown
    - [ ] Language breakdown
    - [ ] Editor breakdown
    - [ ] Project list

API:

- [ ] `GET users/:user/durations`
- [ ] `GET users/:user/goals`
- [ ] `GET users/:user/heartbeats`
- [ ] `GET leaders`
- [ ] `GET users/:user/projects`
- [ ] `GET users/:user/stats/:range`
- [ ] `GET users/:user/summaries`
- [ ] `GET users/:user/teams/:team/members/:member/summaries`
- [ ] `GET users/:user/teams/:team/members`
- [ ] `GET users/:user/teams`
- [ ] `GET users/:user/user_agents`
- [x] `POST users/:user/heartbeats`
- [x] `POST /users/current/heartbeats`

Contact
-------

Author: Michael Zhang

License: MIT
