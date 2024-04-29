// TODO: A Frame has a "layout" property which activates the layout
// If "layout" is true the frame has a Component called LayoutTree or so which manages the nodes layout
// If the Frame size changes we have to recalculate the tree
// If a child changes we have to recalculate the tree
//
// A Node that is part of a Layout gets a NodeId of the Layout so a Component like LayoutId(NodeId)
//
// To go nested we check whether the recomputed child size has changed and the calculate the child
// tree based on whether it was calculated in the current cycle for that we add the cycle count to the layout tree component

// v2
// We have a resource that owns the TaffyTree and since a TaffyTree,
// can be a Tree with multiple root nodes we can use just one Tree
// thus a Resource and not a Component for each Frame.
//
// This way we can update the styles for each node that has changed,
// and then calculate the layout from the most top node
