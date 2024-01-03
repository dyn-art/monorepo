# `@dyn/express-fetch`

```ts
const fetchExpress = createFetchExpress<routes>();

fetchExpress.get('v1/whatever', {
    query: {
        family: z.notEmpty().isString(),
        font_weight: z.optional().isInt(),
    },
    param: {
       key: z.notEmpty().isString(),
    },
    body: {
         
    },
}, (req, res, next) =>{
    // whatever
});
```