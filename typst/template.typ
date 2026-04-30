#let note-template(body) = {
  set page(
    paper: "a4",
    margin: (left: 2.8cm, right: 2.8cm, top: 3cm, bottom: 3cm),
  )

  set text(
    fill: black,
    font: ("New Computer Modern", "Noto Serif CJK SC"),
    lang: "zh",
    region: "cn",
    size: 10.5pt,
  )

  set par(justify: true, leading: 1.2em, first-line-indent: 2em)
  set heading(numbering: "1.1")
  set math.equation(numbering: "(1)", supplement: [式])
  show math.equation: set block(above: 0.9em, below: 0.9em)

  show heading: it => {
    set par(first-line-indent: 0em)

    let title = if it.numbering != none {
      [#numbering(it.numbering, ..counter(heading).at(it.location())) #h(0.4em) #it.body]
    } else {
      it.body
    }

    if it.level == 1 {
      block(above: 2.0em, below: 1.4em)[
        #set text(size: 15pt, weight: "bold")
        #title
      ]
    } else if it.level == 2 {
      block(above: 1.6em, below: 1.1em)[
        #set text(size: 12.5pt, weight: "bold")
        #title
      ]
    } else {
      block(above: 1.2em, below: 0.8em)[
        #set text(size: 11pt, weight: "bold")
        #title
      ]
    }
  }

  body
}

#let pmod(x) = $space (mod #x)$

#let semantic-note(title, body) = block(
  breakable: true,
  inset: (x: 9pt, y: 7pt),
  stroke: (left: (paint: luma(140), thickness: 1pt)),
  above: 0.9em,
  below: 0.9em,
)[
  #text(weight: "bold")[#title]\
  #body
]

#let note(title, body) = semantic-note(title, body)
#let definition(body) = semantic-note("定义", body)
#let theorem(body) = semantic-note("定理", body)
#let proposition(body) = semantic-note("命题", body)
#let lemma(body) = semantic-note("引理", body)
#let corollary(body) = semantic-note("推论", body)
#let example(body) = semantic-note("例", body)
#let proof(body) = semantic-note("证明", body)
#let remark(body) = semantic-note("备注", body)
#let exercise(body) = semantic-note("练习", body)
#let solution(body) = semantic-note("解", body)

#let E = $bb(E)$
#let P = $bb(P)$
#let Var = $upright("Var")$
#let Cov = $upright("Cov")$
#let Normal = $cal(N)$
#let F = $cal(F)$

#let ket(x) = [$|#x chevron.r$]
#let bra(x) = [$chevron.l #x|$]
#let braket(x, y) = [$chevron.l #x|#y chevron.r$]
