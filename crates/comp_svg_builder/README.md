
## Rewrite

### Current

The visual representation of Entities is currently done through a SVG-Bundle. 
An SVG-Bundle is a part of the SVG-Tree made up of multiple SVG-Elements, 
which are the actual SVG tags (e.g., rect, g).

1. **Prepare:** Create SVG-Bundles with only the structural representation of SVG-Elements, without content.
2. **Apply:** Apply changes from Entity Components to the SVG-Bundle. During the update, establish the hierarchy by identifying parent-child relationships among Entities and updating SVGElement children accordingly. An SVGElement can have two types of children: an Entity (WorldContext) or another SVGElement.
3. **Extract & Queue:** Identify changed SVG-Bundles and establish an order for sending events to the frontend based on hierarchy and child indices, ensuring parents exist before their children.

### New?

A more maintainable, and expandable approach to syncing Entities into a fixed XML-Tree
that can be synced with the frontend DOM.

#### Idea 1
In the new approach, we create a SVG-DOM with Nodes referenced in the SVG-Bundle via RC. Treating it like the frontend DOM should simplify synchronization between Rust and the frontend state.

For the SVG-DOM, we could use [RCTree](https://github.com/RazrFalcon/rctree/blob/master/src/lib.rs) and take inspiration from [SvgDom](https://github.com/RazrFalcon/svgdom/tree/master)?