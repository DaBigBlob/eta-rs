;a comment starts with ; and continues to end of line
((E ((A A) E)) ;external capture of eta and omicron

(E (; invoking eta
(P ;capturing lambda application implementation from below
;<---lambda expression begin--->

;NOTE: as we have described,
;   application is of form (E (P (<lambda abstraction>  <applicand>)))
;   abstraction is of form (A (<binding variabel> <body>))
;NOTE: alpha renaming is not a real thing, make sure to name accordingly

;example 1: omega combinator
;uncomment the following to make your stack overflow ☺️
; (E (P (
; (A (y (E (P (y y)))))
; (A (y (E (P (y y)))))
; )))

;example 2: make 4 of the virus 🦠
(E(P ((A (v ((v v) (v v)))) 🦠)))

;<---lambda expression end--->
)

(;this is the lambda application implementation to be used as P
((A (n b)) x)
(E ((n b) x))
)

)))
