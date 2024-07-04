import { ExpressAuth } from '@auth/express';

import { router } from './router';

router.use('/auth/*', ExpressAuth({ providers: [] }));
