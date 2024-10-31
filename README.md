# raylock
##### Swaylock alternitive made in rust   
---
Unfortunatly this is not the most secure desktop locker, as it involves using sway config, and not PAM for key validiation. But it seems to work just fine.

```
# Add this to your sway config:
mode "lock" { }
for_window [title="^raylock$"] sticky enable, fullscreen
```
