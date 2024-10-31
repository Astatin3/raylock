# raylock
##### Swaylock alternitive made in rust   
---
Unfortunatly this is not the most secure desktop locker, as it involves using sway config, and not PAM for key validiation. But it seems to work just fine.

```
# Add this to your sway config:
for_window [title="^raylock$"] sticky enable, floating enable, resize set 1920 px 1080 px, title_format "", border pixel none
mode "lock" {
  bindsym $mod+Shift+q exec true
}
```
