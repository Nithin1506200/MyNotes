# useRef

## `useRef` cant be used for custom react component

## use `useImperativeHandle` for forwording ref

```javascript
function Component(){
    const inputRef=useRef();
    useEffect(()=>{
        inputRef.current.focus()
    },[])
    return (
        <input
        ref={inputRef}
        value={}
        >
    )
}

```
