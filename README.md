# id3-image-hash

Given a mp3 file with a image thumbnail, 
returns hash of the image, 
or if you specific a hash, returns the path of the file if it matches

## Examples

```
[jda@tangent ...Phobia]$ ./id3-image-hash You\ Don\'t\ Know\ Me.mp3
0f00391e88707c3c51779ac04a65c4df3b075828
[jda@tangent ...Phobia]$ ./id3-image-hash You\ Don\'t\ Know\ Me.mp3 -o spam
[jda@tangent ...Phobia]$ ./id3-image-hash You\ Don\'t\ Know\ Me.mp3 -o 0f00391e88707c3c51779ac04a65c4df3b075828
You Don't Know Me.mp3
```