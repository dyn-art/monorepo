import Image from 'next/image';
import Link from 'next/link';
import { Container } from '@dyn/ui';

const Page: React.FC = () => {
	return (
		<div className="flex min-h-screen flex-col bg-[#FEFCF0] pt-4">
			<Container className="min-h-full" size="compact">
				<Link
					className="flex items-center gap-2"
					href="https://inbeta.group"
					rel="noopener noreferrer"
					target="_blank"
				>
					By <Image alt="inbeta.group Logo" height={32} src="/inbetagroup.svg" width={157} />
				</Link>
			</Container>

			<div className="flex flex-grow items-center justify-center">
				<Image alt="dyn.art Logo" height={625} src="/dyndotart-blob.svg" width={709} />
			</div>
		</div>
	);
};

export default Page;
