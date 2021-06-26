# id3-image-hash

Given a mp3 file with a image thumbnail, 
returns hash of the image, 
or if you specific a hash, returns the path of the file if it matches

## Examples

```
[jda@tangent id3-image-hash]$ cd target/debug/
[jda@tangent debug]$ ./id3-image-hash ~/Downloads/...Phobia/You\ Don\'t\ Know\ Me.mp3
0f00391e88707c3c51779ac04a65c4df3b075828
[jda@tangent debug]$ ./id3-image-hash ~/Downloads/...Phobia/You\ Don\'t\ Know\ Me.mp3 -o spam
[jda@tangent debug]$ ./id3-image-hash ~/Downloads/...Phobia/You\ Don\'t\ Know\ Me.mp3 -o 0f00391e88707c3c51779ac04a65c4df3b075828
/home/jda/Downloads/...Phobia/You Don't Know Me.mp3
[jda@tangent debug]$
```