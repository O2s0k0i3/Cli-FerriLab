// The project function defines how your document looks.
// It takes your content and some metadata and formats it.

#import "@preview/physica:0.8.1": *
#import "@preview/tablex:0.0.6": *

// Funciones varias útiles.
#let up(base, exp) = $base^(upright(exp))$
#let down(base, under) = $base_(upright(under))$
#let unidades(valor) = $upright(valor)$
#let uvec(valor) = $arrow(u)_valor$
#let pm = move(dy: -2.3pt, $plus.minus$)

#let project(
  title: "",
  abstract: [],
  author: (),
  date: none,
  bibliography-file: none,
  auto-set-bibliography: false,
  body,
) = {
  // Set the document's basic properties.
  set document(author: author, title: title)
  set page(margin: (left: 20mm, right: 15mm, top: 20mm, bottom: 20mm), 
    numbering: "1", number-align: center)
  set text(font: "New Computer Modern", lang: "es", size: 11pt)
  set par(first-line-indent: 1.5em, justify: true)
  
  // Setting properties.
  align(center)[
    #block(text(weight: 0, 1.75em, title)) 
    #v(3em, weak: true)
    #align(center, author)
    #v(1.5em)
    #align(center, date)
    #v(1.5em)
  ]
  
  // Abstract.
  if abstract != none {
    align(center)[
      #heading(text(0.7em, "Resumen"))] + [#set par(first-line-indent: 1.5em, justify: true)
            #linebreak()

            #abstract // Línea en blanco para que me haga sangría.
      ]
      }
      
  // Main body.
  set par(first-line-indent: 1.5em, justify: true)
  set math.equation(numbering: "(1)")
  
  // Cambia el caption arriba si es tabla, sino lo coloca abajo.
  show figure: it => align(center)[
      #if it.kind == table {
        v(1em)
        block(align(left, it.caption), width: 80%)
        it.body
      } else {
        it.body
        block(align(left, it.caption), width: 80%)
        v(1em)
      }]
    
  // Modifica los títulos.
  show heading: it => {
    set par(first-line-indent: 0pt)
    it.body
    linebreak()
    v(20pt)
  }

  // Personalización de las referencias.
  show ref: it => {
    set text(blue)
    let negro(val) = text(val, black)
    set ref(supplement: it => {
       if it.kind == image {
         negro("Fig.")
       } 
       if it.kind == table {
         negro("Tab.")
        }
      })
    let eq = math.equation
    let el = it.element
    if el != none and el.func() == eq {
      let number = numbering("1", ..counter(eq).at(el.location()))
      text("Ec. ", black)
      link(el.label)[#negro("(")#text(number, blue)#negro(")")]
      } else {
        it
     }
  }
  
  // Personalizar ecuaciones.
  show math.equation: it => {
    set text(weight: 100, size: 11pt)
    set block(below: 20pt, above: 20pt)
    show regex("\d+\.\d+"): it => {show ".": {"," + h(0pt)}
      it}
    it
  }
  
  show regex("\d+\.\d+"): it => {show ".": ","
    it
  }

  // Documento.
  body

  // Bibliography.
  set par(first-line-indent: 0pt, justify: true)
  if bibliography-file != none {
  show bibliography: pad.with(y: 10pt)
  }  
  if auto-set-bibliography {
    bibliography(bibliography-file)
  }
}
