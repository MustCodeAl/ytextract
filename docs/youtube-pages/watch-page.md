# Watch Page (`https://youtube.com/watch?v={ID}`)

The WatchPage mainly contains two JSON blobs holding all the information about a
video.

- InitialData
- PlayerResponse

Interleaved with the data are ***a lot*** of tracking tokens. They are
***everywhere***.

# InitialData

This JSON blob holds the layout information of the website. Basically the whole
webpage is *described* in JSON. It is contained within a `<script>` tag prefixed
by `var ytInitialData = `. Following that follows JSON data ending with a
semicolon and a closing `<script>` tag.

# PlayerResponse

This JSON blob holds all the information that the player needs to display the video. This is:

- Stream data
  - Muxed streams
  - Video and Audio only streams
- Endcard data
- Caption data
- Thumbnail data
- Ad data

