# useImperativeHandle

## use to access from inside

```javascript
    const Login=()=>{
        const emalInputRef=useRef()

        return(
            <Input
            ref={emailInputRef}
            >
        )
    }
```

```javascript
    const Input= React.forwardRef((props,ref)=>{
        const inputRef=UseRef();
        const activate=()=>{
            inputRef.current.focus()
        }
        useImperativeHandle(ref,()=>{  // control from above
            return{
                activate
            }
        })
        return (
            <input ref={InputRef}>
        )

    })
```
