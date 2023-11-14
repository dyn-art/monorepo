import { jsPDF } from 'jspdf';
import React from 'react';
import { Button } from '@/components/primitive';

import { useSVGComposition } from './useSVGComposition';

export const Static: React.FC<TProps> = (props) => {
	const { size } = props;
	const { svgContainerRef, composition } = useSVGComposition({ width: size, height: size });
	const [svg2pdf, setSvg2pdf] = React.useState<any>(null);

	// Dynamically import svg2pdf when the component mounts,
	// because svg2pdf is commonjs
	React.useEffect(() => {
		import('svg2pdf.js').then((module) => {
			setSvg2pdf(() => module.svg2pdf);
		});
	}, []);

	const handleDownloadPDF = React.useCallback(async () => {
		if (svgContainerRef.current != null) {
			const pdf = new jsPDF({
				orientation: 'landscape',
				unit: 'px',
				format: [size, size]
			});

			const svgElement = svgContainerRef.current.querySelector('svg');
			if (svgElement) {
				// Convert and add the SVG content to the PDF
				await svg2pdf(svgElement, pdf, {});

				// Create a Blob from the PDF output and open it in a new tab
				const blob = pdf.output('blob');
				const blobUrl = URL.createObjectURL(blob);
				window.open(blobUrl, '_blank');
			} else {
				console.error('No SVG element found inside the container.');
			}
		}
	}, [svgContainerRef.current, svg2pdf]);

	const handleToString = React.useCallback(async () => {
		if (composition != null) {
			console.log('SVG String: ', composition?.toString());
		}
	}, [composition]);

	return (
		<div className="relative h-full w-full">
			<div ref={svgContainerRef} />
			<div className="absolute left-4 top-4 z-50 flex flex-row gap-x-2">
				<Button onClick={handleDownloadPDF}>To PDF</Button>
				<Button onClick={handleToString}>To String</Button>
			</div>
		</div>
	);
};

type TProps = {
	size: number;
};
