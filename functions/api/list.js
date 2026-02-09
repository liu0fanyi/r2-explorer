export async function onRequestGet(ctx) {
    const { searchParams } = new URL(ctx.request.url);
    const prefix = searchParams.get('prefix') || '';
    const delimiter = '/';

    try {
        const list = await ctx.env.BUCKET.list({
            prefix,
            delimiter,
        });

        const items = [];

        // Folders
        for (const commonPrefix of list.delimitedPrefixes) {
            items.push({
                name: commonPrefix.slice(prefix.length, -1),
                path_type: 'Dir',
                mtime: Date.now(),
                size: null,
            });
        }

        // Files
        for (const object of list.objects) {
            if (object.key === prefix) continue; // Skip the directory itself if it exists as an object
            items.push({
                name: object.key.slice(prefix.length),
                path_type: 'File',
                mtime: object.uploaded.getTime(),
                size: object.size,
            });
        }

        return new Response(JSON.stringify(items), {
            headers: { 'Content-Type': 'application/json' },
        });
    } catch (err) {
        return new Response(JSON.stringify({ error: err.message }), {
            status: 500,
            headers: { 'Content-Type': 'application/json' },
        });
    }
}
