; Anytype assignments
; (assign
;     (identifier) @name) @item

; Function assignments
(assign
    (identifier) @name
    "=" @context
    (function
        (args))) @item

; Let function assignments
(let_assign
    (variable
        (identifier) @name
        ":" @context
        (type (identifier) @name)
    )
    "=" @context
    (function
        (args)
        )) @item
