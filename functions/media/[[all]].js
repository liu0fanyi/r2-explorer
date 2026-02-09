export async function onRequestGet(ctx) {
    const url = new URL(ctx.request.url);
    // Remove /media/ from the path
    const path = decodeURIComponent(url.pathname.replace(/^\/media\//, ''));

    try {
        if (!ctx.env.BUCKET) {
            return new Response("R2 bucket binding 'BUCKET' not found.", {
                status: 500,
                headers: { 'Access-Control-Allow-Origin': '*' }
            });
        }

        const object = await ctx.env.BUCKET.get(path);

        if (!object) {
            return new Response('Not Found', {
                status: 404,
                headers: { 'Access-Control-Allow-Origin': '*' }
            });
        }

        const headers = new Headers();
        object.writeHttpMetadata(headers);
        headers.set('etag', object.httpEtag);
        headers.set('Access-Control-Allow-Origin', '*');

        return new Response(object.body, {
            headers,
        });
    } catch (err) {
        return new Response(err.message, {
            status: 500,
            headers: { 'Access-Control-Allow-Origin': '*' }
        });
    }
}
