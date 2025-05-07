; Anytype assignments
; (assign
;     (identifier) @name) @item

; function assignments
(assign
    (identifier) @name
    "=" @context
    (function
        (args) @name
        "->"? @context
        (type (identifier))? @name
    )
) @item

; let function assignments
(let_assign
    (variable
        (identifier) @name
        ":" @context
        (type (identifier) @name)
    )
    "=" @context
    (function
        (args) @name
        "->"? @context
        (type (identifier))? @name
    )
) @item
