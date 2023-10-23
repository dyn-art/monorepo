import { SVGRenderer, type TSVGRendererOptions } from '../render';
import { Editor, type TEditorConfig } from './Editor';

export function createSVGEditor(config: TSVGEditorConfig) {
	const { renderer: rendererConfig = {}, ...editorConfig } = config;
	const editor = new Editor(editorConfig);
	const svgRenderer = new SVGRenderer(editor, rendererConfig);
	editor.registerRenderer(svgRenderer);
	return editor;
}

type TSVGEditorConfig = {
	renderer?: TSVGRendererOptions;
} & TEditorConfig;
