import React from 'react';

import type { TIconProps } from './types';

export const GoogleIcon = React.forwardRef<SVGSVGElement, TIconProps>((props, ref) => {
	return (
		<svg ref={ref} viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" {...props}>
			<path
				fillRule="evenodd"
				clipRule="evenodd"
				d="M20.16 12.1932C20.16 11.5905 20.1059 11.0109 20.0055 10.4546H12V13.7425H16.5746C16.3775 14.805 15.7786 15.7053 14.8784 16.308V18.4407H17.6255C19.2327 16.9609 20.16 14.7819 20.16 12.1932Z"
				fill="#4285F4"
			/>
			<path
				fillRule="evenodd"
				clipRule="evenodd"
				d="M12 20.5C14.295 20.5 16.2191 19.7389 17.6254 18.4407L14.8784 16.308C14.1173 16.818 13.1436 17.1193 12 17.1193C9.78611 17.1193 7.91224 15.6241 7.24383 13.615H4.40405V15.8173C5.80269 18.5953 8.67724 20.5 12 20.5Z"
				fill="#34A853"
			/>
			<path
				fillRule="evenodd"
				clipRule="evenodd"
				d="M7.24387 13.6151C7.07387 13.105 6.97728 12.5603 6.97728 12C6.97728 11.4398 7.07387 10.895 7.24387 10.385V8.18277H4.40409C3.82841 9.33027 3.5 10.6285 3.5 12C3.5 13.3716 3.82841 14.6698 4.40409 15.8173L7.24387 13.6151Z"
				fill="#FBBC05"
			/>
			<path
				fillRule="evenodd"
				clipRule="evenodd"
				d="M12 6.88072C13.2479 6.88072 14.3684 7.30958 15.2493 8.15186L17.6873 5.7139C16.2152 4.3423 14.2911 3.50003 12 3.50003C8.67724 3.50003 5.80269 5.40481 4.40405 8.18277L7.24383 10.385C7.91224 8.37595 9.78611 6.88072 12 6.88072Z"
				fill="#EA4335"
			/>
		</svg>
	);
});
GoogleIcon.displayName = 'Google Icon';
