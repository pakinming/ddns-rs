```sh

#copy file to device for background execution with 'scp'
on-bg:
  ./ddns-rs > /dev/null 2>&1 &
```