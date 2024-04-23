import { redirect } from 'next/navigation';
import type React from 'react';

const Page: React.FC = () => {
	redirect('/editor/default');
};

export default Page;
