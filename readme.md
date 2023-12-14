# bat-notif

`bat-notif` is a small (linux-only) utility to send a notification on changes to battery state, and a warning on low battery.

it was made to fix some grievances i had with [battery-notify](https://github.com/cdown/battery-notify) and is directly inspired by it.

## config

the config is located at `~/.config/bat-notif.json` and the default config is:

```json
{
    "interval": 10,
    "low_pct": 15
}
```

