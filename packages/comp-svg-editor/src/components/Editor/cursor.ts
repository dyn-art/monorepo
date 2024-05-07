// https://yoksel.github.io/url-encoder/
export const CURSOR = {
	// Cursor / Graphic Default
	default: (): string =>
		`url("data:image/svg+xml,%3Csvg width='32' height='32' viewBox='0 0 32 32' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cg filter='url(%23filter0_d_1731_1364)'%3E%3Cpath fill-rule='evenodd' clip-rule='evenodd' d='M24.4997 15.5535L16.9666 18.6741L12.3594 25.4953L8.59546 8.25513L24.4997 15.5535ZM16.3062 17.8653L21.9999 15.5067L9.99994 9.9999L12.8381 22.9999L16.3062 17.8653Z' fill='white'/%3E%3C/g%3E%3Cpath fill-rule='evenodd' clip-rule='evenodd' d='M22 15.5068L10 10L12.8382 23L16.3062 17.8654L22 15.5068Z' fill='%23363B3E'/%3E%3Cdefs%3E%3Cfilter id='filter0_d_1731_1364' x='5.59546' y='6.25513' width='21.9043' height='23.2402' filterUnits='userSpaceOnUse' color-interpolation-filters='sRGB'%3E%3CfeFlood flood-opacity='0' result='BackgroundImageFix'/%3E%3CfeColorMatrix in='SourceAlpha' type='matrix' values='0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127 0' result='hardAlpha'/%3E%3CfeOffset dy='1'/%3E%3CfeGaussianBlur stdDeviation='1.5'/%3E%3CfeColorMatrix type='matrix' values='0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0.25 0'/%3E%3CfeBlend mode='normal' in2='BackgroundImageFix' result='effect1_dropShadow_1731_1364'/%3E%3CfeBlend mode='normal' in='SourceGraphic' in2='effect1_dropShadow_1731_1364' result='shape'/%3E%3C/filter%3E%3C/defs%3E%3C/svg%3E%0A") 10 10, auto`,
	// Cursor / Grabbing
	grabbing: (): string =>
		`url("data:image/svg+xml,%3Csvg width='32' height='32' viewBox='0 0 32 32' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cg filter='url(%23filter0_d_1731_1339)'%3E%3Cpath fill-rule='evenodd' clip-rule='evenodd' d='M12 24.4V22.3656C11.9596 22.3107 11.899 22.2362 11.8133 22.1423C11.5659 21.871 11.2126 21.5486 10.8277 21.2249C10.4488 20.9064 10.0664 20.6092 9.7765 20.3902C9.63219 20.2811 9.51228 20.1926 9.4291 20.1318C9.38753 20.1014 9.35523 20.078 9.33372 20.0625L9.30979 20.0452L9.30328 20.0406L9.29361 20.0337L9.28407 20.0266C8.87528 19.7228 8.56709 19.299 8.3624 18.8923C8.15755 18.4854 8 17.9836 8 17.4704V13.8667C8 12.1657 9.29036 10.7051 11 10.5988V10.0644C11 8.96558 11.8695 8 13.0335 8C13.7226 8 14.343 8.24351 14.829 8.64426C15.0464 8.56319 15.2831 8.51852 15.5335 8.51852C16.2162 8.51852 16.8316 8.75755 17.3155 9.15172C17.5108 9.07226 17.7199 9.02135 17.9373 9.00386L18.0094 8.99806L18.0817 9.00097C18.7486 9.0278 19.3558 9.27864 19.8343 9.67932C20.0502 9.5995 20.2852 9.55556 20.5335 9.55556C22.1575 9.55556 23.4 10.908 23.4 12.4838V17.2043C23.4 17.6267 23.3134 18.0869 23.1995 18.487C23.0852 18.8886 22.9172 19.3217 22.7011 19.6787L22.6923 19.6932L22.6833 19.7072L22.6806 19.7114L22.6701 19.7279L22.6271 19.7958C22.5895 19.8556 22.535 19.9432 22.4693 20.0511C22.3374 20.2681 22.163 20.5629 21.9902 20.879C21.8153 21.1989 21.6526 21.5212 21.537 21.7969C21.4243 22.0656 21.4043 22.1802 21.4008 22.1797C21.4 22.1796 21.4 22.1743 21.4 22.1642V24.4H12ZM21.6745 19.0573C21.9627 18.5812 22.2 17.7542 22.2 17.2043V12.4838C22.2 11.5685 21.5142 10.8196 20.6464 10.7595C20.6091 10.7569 20.5714 10.7556 20.5335 10.7556C20.073 10.7556 19.7 11.1424 19.7 11.6199V11.9653C19.7 11.847 19.6886 11.7316 19.6667 11.6199C19.5699 11.1239 19.2685 10.7036 18.8579 10.4515C18.6409 10.3184 18.3934 10.2321 18.1295 10.2066C18.0977 10.2035 18.0657 10.2013 18.0335 10.2C17.573 10.237 17.2 10.6239 17.2 11.1014V11.4467C17.2 11.3285 17.1885 11.213 17.1667 11.1014C17.0692 10.6027 16.7647 10.182 16.3502 9.93988C16.1393 9.81672 15.9 9.73978 15.645 9.72232C15.6081 9.7198 15.571 9.71852 15.5335 9.71852C15.073 9.71852 14.7 10.1053 14.7 10.5829V10.9282C14.7 10.81 14.6885 10.6945 14.6667 10.5829C14.5694 10.0851 14.2658 9.66495 13.8524 9.42265C13.6106 9.2809 13.3312 9.2 13.0335 9.2C12.573 9.2 12.2 9.58681 12.2 10.0644V14.9037H11.2V11.7926C11.1325 11.7926 11.0658 11.7961 11 11.8028C10.627 11.8412 10.2842 11.9859 10 12.2073C9.51423 12.5857 9.2 13.1882 9.2 13.8667V17.4704C9.2 18.0277 9.5581 18.7352 9.99985 19.0634C9.99985 19.0634 13.2 21.3447 13.2 22.1635V23.2H20.2V22.1635C20.2 21.3447 21.6745 19.0573 21.6745 19.0573ZM12.06 22.4589C12.0596 22.4588 12.0561 22.4529 12.0511 22.4416C12.058 22.4534 12.0605 22.459 12.06 22.4589Z' fill='%23363B3E'/%3E%3C/g%3E%3Cpath fill-rule='evenodd' clip-rule='evenodd' d='M13.2 23.2V22.1634C13.2 21.3447 9.9998 19.0634 9.9998 19.0634C9.55805 18.7351 9.19995 18.0277 9.19995 17.4703V13.8666C9.19995 12.7212 10.0955 11.7925 11.2 11.7925V14.9037H12.2V10.0643C12.2 9.58677 12.573 9.19995 13.0335 9.19995C13.954 9.19995 14.7 9.97358 14.7 10.9282V10.5828C14.7 10.1053 15.073 9.71847 15.5335 9.71847C16.454 9.71847 17.2 10.4921 17.2 11.4467V11.1014C17.2 10.6238 17.573 10.237 18.0335 10.2C18.954 10.237 19.7 11.0106 19.7 11.9652V11.6199C19.7 11.1423 20.073 10.7555 20.5335 10.7555C21.454 10.7555 22.2 11.5291 22.2 12.4837V17.2043C22.2 17.7542 21.9627 18.5812 21.6745 19.0572C21.6745 19.0572 20.2 21.3447 20.2 22.1634V23.2H13.2Z' fill='white'/%3E%3Cdefs%3E%3Cfilter id='filter0_d_1731_1339' x='5' y='6' width='21.3999' height='22.3999' filterUnits='userSpaceOnUse' color-interpolation-filters='sRGB'%3E%3CfeFlood flood-opacity='0' result='BackgroundImageFix'/%3E%3CfeColorMatrix in='SourceAlpha' type='matrix' values='0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127 0' result='hardAlpha'/%3E%3CfeOffset dy='1'/%3E%3CfeGaussianBlur stdDeviation='1.5'/%3E%3CfeColorMatrix type='matrix' values='0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0.2 0'/%3E%3CfeBlend mode='normal' in2='BackgroundImageFix' result='effect1_dropShadow_1731_1339'/%3E%3CfeBlend mode='normal' in='SourceGraphic' in2='effect1_dropShadow_1731_1339' result='shape'/%3E%3C/filter%3E%3C/defs%3E%3C/svg%3E%0A") 16 16, auto`,
	// Cursor / Resize NS
	resize: (rotation = 0): string =>
		`url("data:image/svg+xml,%3Csvg width='32' height='32' viewBox='0 0 32 32' transform='rotate(${rotation}, 0, 0)' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cg filter='url(%23filter0_d_1731_1348)'%3E%3Cpath fill-rule='evenodd' clip-rule='evenodd' d='M15.0001 20.5V11.5H12.0001L16.0001 6.49995L20.0001 11.5H17.0001V20.5H20.0001L16.0001 25.5L12.0001 20.5H15.0001ZM18.0001 19.5H22.0807L16.0001 27.1007L9.91943 19.5H14.0001V12.5H9.91943L16.0001 4.89917L22.0807 12.5H18.0001V19.5Z' fill='white'/%3E%3C/g%3E%3Cpath fill-rule='evenodd' clip-rule='evenodd' d='M17 20.5V11.5H20L16 6.5L12 11.5H15V20.5H12L16 25.5L20 20.5H17Z' fill='%23363B3E'/%3E%3Cdefs%3E%3Cfilter id='filter0_d_1731_1348' x='6.91943' y='2.89917' width='18.1611' height='28.2017' filterUnits='userSpaceOnUse' color-interpolation-filters='sRGB'%3E%3CfeFlood flood-opacity='0' result='BackgroundImageFix'/%3E%3CfeColorMatrix in='SourceAlpha' type='matrix' values='0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127 0' result='hardAlpha'/%3E%3CfeOffset dy='1'/%3E%3CfeGaussianBlur stdDeviation='1.5'/%3E%3CfeColorMatrix type='matrix' values='0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0.25 0'/%3E%3CfeBlend mode='normal' in2='BackgroundImageFix' result='effect1_dropShadow_1731_1348'/%3E%3CfeBlend mode='normal' in='SourceGraphic' in2='effect1_dropShadow_1731_1348' result='shape'/%3E%3C/filter%3E%3C/defs%3E%3C/svg%3E%0A") 16 16, auto`,
	// Cursor / Rotate Top Left
	rotate: (rotation = 0): string =>
		`url("data:image/svg+xml,%3Csvg width='32' height='32' viewBox='0 0 32 32' transform='rotate(${rotation}, 0, 0)' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cg filter='url(%23filter0_d_1731_1356)'%3E%3Cpath fill-rule='evenodd' clip-rule='evenodd' d='M10.5962 24.7291L13.9216 20.5723L11.4276 20.5723L11.4276 15.9999C11.4276 15.9999 11.2314 13.2014 12.228 12.2048C13.2246 11.2081 16 11.4275 16 11.4275L20.5724 11.4275L20.5724 13.9215L24.7291 10.5961L20.5724 7.27073L20.5724 9.76478L15.1686 9.76478C15.1686 9.76478 11.9499 9.67263 10.8084 10.8141C9.6669 11.9556 9.76486 15.1685 9.76486 15.1685L9.76486 20.5723L7.27081 20.5723L10.5962 24.7291ZM8.76486 19.5723L5.19019 19.5723L10.5962 26.3298L16.0022 19.5723L12.4276 19.5723L12.4276 15.9649L12.4252 15.9309L12.425 15.928L12.4239 15.9108C12.423 15.8944 12.4216 15.8685 12.4201 15.8342C12.4171 15.7654 12.4137 15.6634 12.4124 15.5369C12.4098 15.2822 12.416 14.9366 12.4499 14.5673C12.4841 14.1939 12.5443 13.823 12.6397 13.5078C12.7391 13.1791 12.8517 12.9953 12.9351 12.9119C13.018 12.829 13.1994 12.718 13.5241 12.621C13.8359 12.5279 14.2028 12.4703 14.5726 12.4388C14.9383 12.4076 15.2804 12.404 15.5327 12.4086C15.6579 12.4109 15.7588 12.4151 15.8268 12.4186C15.8607 12.4204 15.8863 12.422 15.9024 12.4231L15.9194 12.4242L15.9212 12.4244L15.9222 12.4244L15.9605 12.4275L19.5724 12.4275L19.5724 16.0021L26.3299 10.5961L19.5724 5.19011L19.5724 8.76478L15.1806 8.76478L15.1488 8.76416C15.1207 8.7637 15.0811 8.76321 15.0313 8.76305C14.9317 8.76273 14.7908 8.76369 14.6191 8.76863C14.2773 8.77847 13.8056 8.80431 13.2913 8.86927C12.7808 8.93374 12.2033 9.03976 11.6591 9.21856C11.1286 9.39286 10.545 9.66335 10.1013 10.107C9.65749 10.5508 9.38735 11.1342 9.21352 11.6646C9.03524 12.2085 8.92999 12.7855 8.86628 13.2954C8.80208 13.8092 8.77701 14.2802 8.76775 14.6216C8.7631 14.7931 8.76239 14.9338 8.76289 15.0332C8.76314 15.083 8.76369 15.1226 8.7642 15.1506L8.76486 15.1812L8.76486 19.5723Z' fill='white'/%3E%3C/g%3E%3Cpath d='M7.27075 20.5725L9.7648 20.5725L9.7648 15.1687C9.7648 15.1687 9.66685 11.9558 10.8083 10.8143C11.9498 9.67282 15.1686 9.76497 15.1686 9.76497L20.5723 9.76497L20.5723 7.27093L24.7291 10.5963L20.5723 13.9217L20.5723 11.4277L15.9999 11.4277C15.9999 11.4277 13.2246 11.2083 12.228 12.205C11.2313 13.2016 11.4275 16.0001 11.4275 16.0001L11.4275 20.5725L13.9215 20.5725L10.5961 24.7292L7.27075 20.5725Z' fill='%23363B3E'/%3E%3Cdefs%3E%3Cfilter id='filter0_d_1731_1356' x='2.19019' y='3.19019' width='27.1396' height='27.1396' filterUnits='userSpaceOnUse' color-interpolation-filters='sRGB'%3E%3CfeFlood flood-opacity='0' result='BackgroundImageFix'/%3E%3CfeColorMatrix in='SourceAlpha' type='matrix' values='0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127 0' result='hardAlpha'/%3E%3CfeOffset dy='1'/%3E%3CfeGaussianBlur stdDeviation='1.5'/%3E%3CfeColorMatrix type='matrix' values='0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0.25 0'/%3E%3CfeBlend mode='normal' in2='BackgroundImageFix' result='effect1_dropShadow_1731_1356'/%3E%3CfeBlend mode='normal' in='SourceGraphic' in2='effect1_dropShadow_1731_1356' result='shape'/%3E%3C/filter%3E%3C/defs%3E%3C/svg%3E%0A") 16 16, auto`,
	// Cursor / Crosshair
	crosshair: (): string =>
		`url("data:image/svg+xml,%3Csvg width='32' height='32' viewBox='0 0 32 32' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cg filter='url(%23filter0_d_1731_1372)'%3E%3Cpath fill-rule='evenodd' clip-rule='evenodd' d='M18 7V14H25V18H18V25H14V18H7V14H14V7H18ZM17 15H24V17H17V24H15V17H8V15H15V8H17V15Z' fill='white'/%3E%3C/g%3E%3Cpath fill-rule='evenodd' clip-rule='evenodd' d='M17 15V8H15V15H8V17H15V24H17V17H24V15H17V15Z' fill='%23363B3E'/%3E%3Cdefs%3E%3Cfilter id='filter0_d_1731_1372' x='4' y='5' width='24' height='24' filterUnits='userSpaceOnUse' color-interpolation-filters='sRGB'%3E%3CfeFlood flood-opacity='0' result='BackgroundImageFix'/%3E%3CfeColorMatrix in='SourceAlpha' type='matrix' values='0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127 0' result='hardAlpha'/%3E%3CfeOffset dy='1'/%3E%3CfeGaussianBlur stdDeviation='1.5'/%3E%3CfeColorMatrix type='matrix' values='0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0.25 0'/%3E%3CfeBlend mode='normal' in2='BackgroundImageFix' result='effect1_dropShadow_1731_1372'/%3E%3CfeBlend mode='normal' in='SourceGraphic' in2='effect1_dropShadow_1731_1372' result='shape'/%3E%3C/filter%3E%3C/defs%3E%3C/svg%3E%0A") 16 16, auto`
};
