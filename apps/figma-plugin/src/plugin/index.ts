figma.showUI(__html__);

figma.ui.onmessage = (msg) => {
	console.log('Hello: ', { msg });
};
