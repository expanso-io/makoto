export default {
  async fetch(request) {
    const url = new URL(request.url);
    const target = "https://usemakoto.dev";

    // 301 Redirect: dpl.dev/foo -> usemakoto.dev/foo
    return Response.redirect(target + url.pathname + url.search, 301);
  },
};
