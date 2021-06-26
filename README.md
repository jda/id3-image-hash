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

## y tho?
~13 years ago my music library got _somehow_ contaminated with ~8GB of Trance remix tracks with bad tags but they all used the same trippy embedded album art.
This made my browsing my music library rather annoying, but I couldn't clean it up based on tags.

Last night I got sufficiently annoyed and set out to find a way to get those tracks out of my library. 
I found [id3-image](https://github.com/AndrewRadev/id3-image) which was close, but not quite what I needed: it exported the images to filesystem, which I could have shell scripted my way around, but it seemed kinda icky to drop jpgs or whatever all over the filesystem to solve this. Instead, I hacked up `id3-image-hash` to handle getting a hash from a file without incuring filesystem writes for every mp3. 
