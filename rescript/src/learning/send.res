type audio
type actions
@send external play: actions => unit = "play"
@new external audio: string => actions = "Audio"

let song = audio("fs")
song->play
