# VxTwitterBot

Telegram bot ([Telegram@vxttbot](https://t.me/vxttbot)) for generating Twitter link previews
using [VxTwitter](https://github.com/dylanpdx/BetterTwitFix) & removing tracker parameters in the url.
Works in inline mode.

No personal information is collected by this bot.

## Nginx setup for webhook

Change `webhook_addr` & `webhook_url` in [src/main.rs](src/main.rs) first.

```
# nginx.conf
location /vxtwitterbot/ {
    allow 149.154.160.0/20;
    allow 91.108.4.0/22;
    deny all;
    proxy_pass http://127.0.0.1:12221;
}
```