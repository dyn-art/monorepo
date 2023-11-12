import { jsPDF } from 'jspdf';
import React from 'react';
import { Button } from '@/components/primitive';

import { useSVGComposition } from './useSVGComposition';

export const Static: React.FC<TProps> = (props) => {
	const { size } = props;
	const svgContainerRef = useSVGComposition({ width: size, height: size });
	const [svg2pdf, setSvg2pdf] = React.useState<any>(null);

	// Dynamically import svg2pdf when the component mounts,
	// because svg2pdf is commonjs
	React.useEffect(() => {
		import('svg2pdf.js').then((module) => {
			setSvg2pdf(() => module.svg2pdf);
		});
	}, []);

	const handleDownloadPDF = React.useCallback(async () => {
		if (svgContainerRef.current) {
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

	return (
		<div className="relative">
			<div ref={svgContainerRef} />
			<Button onClick={handleDownloadPDF} className="absolute left-4 top-4 z-50">
				To PDF
			</Button>
		</div>
	);
};

type TProps = {
	size: number;
};
