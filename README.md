WaT
===

[![Travis](https://img.shields.io/travis/iptq/wat.svg)](https://travis-ci.org/iptq/wat)
[![GoReportCard](https://goreportcard.com/badge/github.com/iptq/wat)](https://goreportcard.com/report/github.com/iptq/wat)

Drop-in WakaTime replacement.

How To Use
----------

First, you're going to need to set up the WaT server. You should host it from a location that your computer can reach, since API calls will be made to this server. If you're just serving to yourself, running it on localhost should do the trick. Otherwise, make sure at least the API is reachable.

Under your `~/.wakatime.cfg`, add a line indicating where your `api_url` is pointing to. For example,

    api_url=https://wat.mzhang.me/api/v1/heartbeats.bulk

Since most WakaTime editor plugins respect this configuration file, they should be pointed at the correct place as long as you set this value.

Why?
----

I've nothing against WakaTime, and I'm personally very grateful for the community they've built around tracking coding statistics. But at the end of the day, I'd like to keep my own data, and hopefully you do too. This project gives you the option to host a WakaTime-compatible server while allowing you to continue using the great plugins that people have developed for WakaTime.

Roadmap
-------

General usability:

- [ ] Web interface
  - [ ] Registration

API:

- [ ] `GET users/:user/durations`
- [ ] `GET users/:user/goals`
- [ ] `GET users/:user/heartbeats`
- [x] `POST users/:user/heartbeats`
- [ ] `GET leaders`
- [ ] `GET users/:user/projects`
- [ ] `GET users/:user/stats/:range`
- [ ] `GET users/:user/summaries`
- [ ] `GET users/:user/teams/:team/members/:member/summaries`
- [ ] `GET users/:user/teams/:team/members`
- [ ] `GET users/:user/teams`
- [ ] `GET users/:user/user_agents`

Contact
-------

Author: Michael Zhang

License: MIT
