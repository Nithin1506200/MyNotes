type audio
type actions
@send external play: actions => unit = "play"
@new external audio: string => actions = "Audio"

let song = audio("fs")
song->play
@val external document: Dom.element = "document"

@send external getElementById: (Dom.element, string) => Dom.element = "getElementById"
@send external isEqualNode: (Dom.element, Dom.element) => bool = "isEqualNode"

let a = document->getElementById("root")
let b = document->getElementById("root")
@val
external alert: string => unit = "alert"
alert("Hello, world!")

//

isEqualNode(a, b)->Js.log
