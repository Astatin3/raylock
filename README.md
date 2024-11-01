# raylock
##### Swaylock alternitive made in rust   
---
Unfortunatly this is not the most secure desktop locker, as it involves using sway config, and not PAM for key validiation. But it seems to work just fine.

```
# Add this to your sway config:
for_window [title="^raylock$"] sticky enable, fullscreen
mode "lock" {
 bindsym XF86MonBrightnessUp exec brightnessctl s +5%
 bindsym XF86MonBrightnessDown exec brightnessctl s 5%-
}
```
