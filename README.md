# Description
`o` opens files based on their mime type from `libmagic`

# Config File
```cat ~/.config/o.conf
{
  "mime_type_to_program": {
    "image/png": "imv",
    "image/jpeg": "imv",
    "text/plain": "nvim"
  }
}
```

# Usage
```
$ o mydata.txt
$ o mypicture.jpg
$ o mymovie.mp4
```
