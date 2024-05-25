import React from 'react';
import {
	Input,
	Select,
	SelectContent,
	SelectGroup,
	SelectItem,
	SelectLabel,
	SelectTrigger,
	SelectValue,
	TabsContent
} from '@/components/primitive';

import type { TImagePaint, TImageScaleMode, TPaint } from './types';

export const ImagePaintTab: React.FC<TProps> = (props) => {
	const { paint, onPaintUpdate } = props;
	const [selectedFile, setSelectedFile] = React.useState<File | null>(null);
	const [preview, setPreview] = React.useState<string | null>(null);
	const [scaleMode, setScaleMode] = React.useState<TImageScaleMode>({ type: 'Fill' });

	React.useEffect(() => {
		if (selectedFile) {
			const reader = new FileReader();
			reader.onloadend = () => {
				const content = Array.from(new Uint8Array(reader.result as ArrayBuffer));
				const imagePaint: TImagePaint = {
					type: 'Image',
					scaleMode,
					content,
					opacity: 1
				};
				onPaintUpdate(imagePaint);
			};
			reader.readAsArrayBuffer(selectedFile);
		}
	}, [selectedFile, scaleMode, onPaintUpdate]);

	const handleFileChange = React.useCallback((event: React.ChangeEvent<HTMLInputElement>) => {
		const file = event.target.files?.[0];
		if (file) {
			setSelectedFile(file);
			const reader = new FileReader();
			reader.onloadend = () => {
				setPreview(reader.result as string);
			};
			reader.readAsDataURL(file);
		}
	}, []);

	const handleScaleModeChange = React.useCallback((value: 'Fit' | 'Fill') => {
		const newScaleMode: TImageScaleMode = { type: value };
		setScaleMode(newScaleMode);
	}, []);

	return (
		<TabsContent className="mt-0 flex flex-wrap gap-1" value="Image">
			<div className="flex flex-col gap-4">
				<div>
					<label htmlFor="scaleMode">Scale Mode:</label>
					<Select onValueChange={handleScaleModeChange} value={scaleMode.type}>
						<SelectTrigger className="w-[180px]">
							<SelectValue placeholder="Select a mode" />
						</SelectTrigger>
						<SelectContent>
							<SelectGroup>
								<SelectLabel>Modes</SelectLabel>
								<SelectItem value="Fill">Fill</SelectItem>
								<SelectItem value="Fit">Fit</SelectItem>
								{/* <SelectItem value="Crop">Crop</SelectItem>
								<SelectItem value="Tile">Tile</SelectItem> */}
							</SelectGroup>
						</SelectContent>
					</Select>
				</div>
				<div className="relative">
					{preview && paint.type === 'Image' ? (
						<div className="group relative mt-4">
							<img alt="Preview" className="h-auto max-w-full" src={preview} />
							<label
								className="absolute inset-0 flex items-center justify-center bg-black bg-opacity-50 text-white opacity-0 transition-opacity group-hover:opacity-100"
								htmlFor="fileUpload"
							>
								Select Image
							</label>
						</div>
					) : (
						<div className="group relative mt-4 flex h-48 w-full items-center justify-center border border-dashed border-gray-300">
							<span className="text-gray-500">No image selected</span>
							<label
								className="absolute inset-0 flex items-center justify-center bg-black bg-opacity-50 text-white opacity-0 transition-opacity group-hover:opacity-100"
								htmlFor="fileUpload"
							>
								Select Image
							</label>
						</div>
					)}
					<Input
						className="absolute inset-0 opacity-0"
						id="fileUpload"
						onChange={handleFileChange}
						type="file"
					/>
				</div>
			</div>
		</TabsContent>
	);
};

interface TProps {
	paint: TPaint;
	onPaintUpdate: (paint: TPaint) => void;
}
