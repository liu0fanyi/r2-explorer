export async function onRequestGet(ctx) {
    const url = new URL(ctx.request.url);
    // Remove /media/ from the path
    const path = decodeURIComponent(url.pathname.replace(/^\/media\//, ''));

    try {
        const object = await ctx.env.BUCKET.get(path);

        if (!object) {
            return new Response('Not Found', { status: 404 });
        }

        const headers = new Headers();
        object.writeHttpMetadata(headers);
        headers.set('etag', object.httpEtag);

        return new Response(object.body, {
            headers,
        });
    } catch (err) {
        return new Response(err.message, { status: 500 });
    }
}
