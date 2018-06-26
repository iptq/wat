WaT
===

[![Travis](https://img.shields.io/travis/iptq/wat.svg)](https://travis-ci.org/iptq/wat)

Drop-in WakaTime replacement.

How To Use
----------

Under your `~/.wakatime.cfg`, add a line indicating where your `api_url` is pointing to. For example,

    api_url=https://wat.mzhang.me/api/v1/heartbeats.bulk

Then, all of your WakaTime-enabled editors should be pointed at the correct place!

Roadmap
-------

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
