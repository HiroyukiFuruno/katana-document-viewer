# KatanA DrawIo Scroll Performance Fixture

This fixture keeps the large-window scroll performance artifact independent from
external Mermaid or PlantUML tools while still rendering real diagram surfaces.

## 1. Intake Flow

```drawio
<mxGraphModel dx="900" dy="700" grid="1" gridSize="10" page="1" pageWidth="850" pageHeight="900">
  <root>
    <mxCell id="0"/>
    <mxCell id="1" parent="0"/>
    <mxCell id="start" value="Start" style="ellipse;whiteSpace=wrap;html=1;fillColor=#d5e8d4;strokeColor=#82b366;" parent="1" vertex="1">
      <mxGeometry x="150" y="60" width="120" height="60" as="geometry"/>
    </mxCell>
    <mxCell id="validate" value="Validate" style="rounded=1;whiteSpace=wrap;html=1;fillColor=#dae8fc;strokeColor=#6c8ebf;" parent="1" vertex="1">
      <mxGeometry x="150" y="180" width="120" height="60" as="geometry"/>
    </mxCell>
    <mxCell id="route" value="Route" style="rhombus;whiteSpace=wrap;html=1;fillColor=#fff2cc;strokeColor=#d6b656;" parent="1" vertex="1">
      <mxGeometry x="160" y="310" width="100" height="90" as="geometry"/>
    </mxCell>
    <mxCell id="done" value="Done" style="ellipse;whiteSpace=wrap;html=1;fillColor=#f8cecc;strokeColor=#b85450;" parent="1" vertex="1">
      <mxGeometry x="150" y="500" width="120" height="60" as="geometry"/>
    </mxCell>
    <mxCell id="edge-start-validate" edge="1" source="start" target="validate" parent="1" style="endArrow=block;html=1;rounded=0;">
      <mxGeometry relative="1" as="geometry"/>
    </mxCell>
    <mxCell id="edge-validate-route" edge="1" source="validate" target="route" parent="1" style="endArrow=block;html=1;rounded=0;">
      <mxGeometry relative="1" as="geometry"/>
    </mxCell>
    <mxCell id="edge-route-done" edge="1" source="route" target="done" parent="1" style="endArrow=block;html=1;rounded=0;">
      <mxGeometry relative="1" as="geometry"/>
    </mxCell>
  </root>
</mxGraphModel>
```

## 2. Processing Flow

```drawio
<mxGraphModel dx="900" dy="700" grid="1" gridSize="10" page="1" pageWidth="850" pageHeight="900">
  <root>
    <mxCell id="0"/>
    <mxCell id="1" parent="0"/>
    <mxCell id="queue" value="Queue" style="rounded=1;whiteSpace=wrap;html=1;fillColor=#dae8fc;strokeColor=#6c8ebf;" parent="1" vertex="1">
      <mxGeometry x="80" y="80" width="150" height="70" as="geometry"/>
    </mxCell>
    <mxCell id="worker" value="Worker" style="rounded=1;whiteSpace=wrap;html=1;fillColor=#d5e8d4;strokeColor=#82b366;" parent="1" vertex="1">
      <mxGeometry x="330" y="80" width="150" height="70" as="geometry"/>
    </mxCell>
    <mxCell id="cache" value="Cache" style="shape=cylinder;whiteSpace=wrap;html=1;boundedLbl=1;backgroundOutline=1;size=15;fillColor=#fff2cc;strokeColor=#d6b656;" parent="1" vertex="1">
      <mxGeometry x="330" y="240" width="150" height="90" as="geometry"/>
    </mxCell>
    <mxCell id="surface" value="Surface" style="rounded=1;whiteSpace=wrap;html=1;fillColor=#f8cecc;strokeColor=#b85450;" parent="1" vertex="1">
      <mxGeometry x="580" y="240" width="150" height="70" as="geometry"/>
    </mxCell>
    <mxCell id="edge-queue-worker" edge="1" source="queue" target="worker" parent="1" style="endArrow=block;html=1;rounded=0;">
      <mxGeometry relative="1" as="geometry"/>
    </mxCell>
    <mxCell id="edge-worker-cache" edge="1" source="worker" target="cache" parent="1" style="endArrow=block;html=1;rounded=0;">
      <mxGeometry relative="1" as="geometry"/>
    </mxCell>
    <mxCell id="edge-cache-surface" edge="1" source="cache" target="surface" parent="1" style="endArrow=block;html=1;rounded=0;">
      <mxGeometry relative="1" as="geometry"/>
    </mxCell>
  </root>
</mxGraphModel>
```

## 3. Presentation Flow

```drawio
<mxGraphModel dx="900" dy="700" grid="1" gridSize="10" page="1" pageWidth="850" pageHeight="900">
  <root>
    <mxCell id="0"/>
    <mxCell id="1" parent="0"/>
    <mxCell id="scene" value="Scene" style="rounded=1;whiteSpace=wrap;html=1;fillColor=#dae8fc;strokeColor=#6c8ebf;" parent="1" vertex="1">
      <mxGeometry x="80" y="90" width="140" height="60" as="geometry"/>
    </mxCell>
    <mxCell id="band" value="Presented Band" style="rounded=1;whiteSpace=wrap;html=1;fillColor=#d5e8d4;strokeColor=#82b366;" parent="1" vertex="1">
      <mxGeometry x="310" y="90" width="170" height="60" as="geometry"/>
    </mxCell>
    <mxCell id="scroll" value="Wheel Scroll" style="rounded=1;whiteSpace=wrap;html=1;fillColor=#fff2cc;strokeColor=#d6b656;" parent="1" vertex="1">
      <mxGeometry x="560" y="90" width="150" height="60" as="geometry"/>
    </mxCell>
    <mxCell id="present" value="Present" style="ellipse;whiteSpace=wrap;html=1;fillColor=#f8cecc;strokeColor=#b85450;" parent="1" vertex="1">
      <mxGeometry x="335" y="260" width="120" height="70" as="geometry"/>
    </mxCell>
    <mxCell id="edge-scene-band" edge="1" source="scene" target="band" parent="1" style="endArrow=block;html=1;rounded=0;">
      <mxGeometry relative="1" as="geometry"/>
    </mxCell>
    <mxCell id="edge-band-scroll" edge="1" source="band" target="scroll" parent="1" style="endArrow=block;html=1;rounded=0;">
      <mxGeometry relative="1" as="geometry"/>
    </mxCell>
    <mxCell id="edge-scroll-present" edge="1" source="scroll" target="present" parent="1" style="endArrow=block;html=1;rounded=0;">
      <mxGeometry relative="1" as="geometry"/>
    </mxCell>
  </root>
</mxGraphModel>
```

## 4. Cache Flow

```drawio
<mxGraphModel dx="900" dy="700" grid="1" gridSize="10" page="1" pageWidth="850" pageHeight="900">
  <root>
    <mxCell id="0"/>
    <mxCell id="1" parent="0"/>
    <mxCell id="hash" value="Source Hash" style="rounded=1;whiteSpace=wrap;html=1;fillColor=#dae8fc;strokeColor=#6c8ebf;" parent="1" vertex="1">
      <mxGeometry x="90" y="90" width="150" height="60" as="geometry"/>
    </mxCell>
    <mxCell id="dpi" value="DPI" style="rounded=1;whiteSpace=wrap;html=1;fillColor=#fff2cc;strokeColor=#d6b656;" parent="1" vertex="1">
      <mxGeometry x="340" y="90" width="120" height="60" as="geometry"/>
    </mxCell>
    <mxCell id="surface" value="Retina Surface" style="rounded=1;whiteSpace=wrap;html=1;fillColor=#d5e8d4;strokeColor=#82b366;" parent="1" vertex="1">
      <mxGeometry x="560" y="90" width="160" height="60" as="geometry"/>
    </mxCell>
    <mxCell id="edge-hash-dpi" edge="1" source="hash" target="dpi" parent="1" style="endArrow=block;html=1;rounded=0;">
      <mxGeometry relative="1" as="geometry"/>
    </mxCell>
    <mxCell id="edge-dpi-surface" edge="1" source="dpi" target="surface" parent="1" style="endArrow=block;html=1;rounded=0;">
      <mxGeometry relative="1" as="geometry"/>
    </mxCell>
  </root>
</mxGraphModel>
```

## 5. Interaction Flow

```drawio
<mxGraphModel dx="900" dy="700" grid="1" gridSize="10" page="1" pageWidth="850" pageHeight="900">
  <root>
    <mxCell id="0"/>
    <mxCell id="1" parent="0"/>
    <mxCell id="pointer" value="Pointer" style="ellipse;whiteSpace=wrap;html=1;fillColor=#d5e8d4;strokeColor=#82b366;" parent="1" vertex="1">
      <mxGeometry x="120" y="90" width="120" height="60" as="geometry"/>
    </mxCell>
    <mxCell id="kuc" value="KUC Hit Surface" style="rounded=1;whiteSpace=wrap;html=1;fillColor=#dae8fc;strokeColor=#6c8ebf;" parent="1" vertex="1">
      <mxGeometry x="330" y="90" width="170" height="60" as="geometry"/>
    </mxCell>
    <mxCell id="command" value="Typed Command" style="rounded=1;whiteSpace=wrap;html=1;fillColor=#f8cecc;strokeColor=#b85450;" parent="1" vertex="1">
      <mxGeometry x="590" y="90" width="160" height="60" as="geometry"/>
    </mxCell>
    <mxCell id="edge-pointer-kuc" edge="1" source="pointer" target="kuc" parent="1" style="endArrow=block;html=1;rounded=0;">
      <mxGeometry relative="1" as="geometry"/>
    </mxCell>
    <mxCell id="edge-kuc-command" edge="1" source="kuc" target="command" parent="1" style="endArrow=block;html=1;rounded=0;">
      <mxGeometry relative="1" as="geometry"/>
    </mxCell>
  </root>
</mxGraphModel>
```

## 6. Scrolling Flow

```drawio
<mxGraphModel dx="900" dy="700" grid="1" gridSize="10" page="1" pageWidth="850" pageHeight="900">
  <root>
    <mxCell id="0"/>
    <mxCell id="1" parent="0"/>
    <mxCell id="before" value="Before Frame" style="rounded=1;whiteSpace=wrap;html=1;fillColor=#dae8fc;strokeColor=#6c8ebf;" parent="1" vertex="1">
      <mxGeometry x="90" y="90" width="160" height="60" as="geometry"/>
    </mxCell>
    <mxCell id="scroll" value="Scroll Delta" style="rhombus;whiteSpace=wrap;html=1;fillColor=#fff2cc;strokeColor=#d6b656;" parent="1" vertex="1">
      <mxGeometry x="350" y="70" width="120" height="100" as="geometry"/>
    </mxCell>
    <mxCell id="after" value="After Frame" style="rounded=1;whiteSpace=wrap;html=1;fillColor=#d5e8d4;strokeColor=#82b366;" parent="1" vertex="1">
      <mxGeometry x="580" y="90" width="160" height="60" as="geometry"/>
    </mxCell>
    <mxCell id="edge-before-scroll" edge="1" source="before" target="scroll" parent="1" style="endArrow=block;html=1;rounded=0;">
      <mxGeometry relative="1" as="geometry"/>
    </mxCell>
    <mxCell id="edge-scroll-after" edge="1" source="scroll" target="after" parent="1" style="endArrow=block;html=1;rounded=0;">
      <mxGeometry relative="1" as="geometry"/>
    </mxCell>
  </root>
</mxGraphModel>
```

## 7. Tail Content

The artifact scrolls through text and several rendered DrawIo surfaces so the
frame budget covers the same cached presented-band path used by the interactive
viewer when a loaded diagram is present.

Additional tail text keeps the large-window scroll range above the measured
wheel deltas. The test should not depend on external diagram tools, but it
must still include real loaded diagram surfaces throughout the document.
