This TTF file is a modified version of the data included in [ttf-parser]'s repository for testing purposes.
Modification is done simply by `ttx` command, provided by fonttools.

``` sh
ttx -o tmp.ttx demo.ttf

# (edit tmp.ttx on a text editor)

ttx -o test.ttf tmp.ttx
```

It contains a glyph data for 'A'.

``` xml
<TTGlyph name="A" xMin="0" yMin="0" xMax="100" yMax="100">
    <contour>
    <pt x="0" y="0" on="1"/>
    <pt x="100" y="100" on="1"/>
    <pt x="0" y="100" on="1"/>
    </contour>
    <instructions/>
</TTGlyph>
```
