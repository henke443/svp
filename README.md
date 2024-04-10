# .svp

This is a super quick (like an hour or two) experiment I did when thinking about the .PDF format.


Basically I don't like PDF. It's super bloated (how often do you use U3D or PRC 3D graphics in your PDF?), prone to being insecure (historically at least) and convoluted to work with in general. 

Yet, it's widely used when someone wants to send a document that should automatically be interpreted by the users device/browser as a document consisting of a number of pages and which can contain text and vector graphics. While a .html can technically fulfill the same purpose, it's much harder to make it behave like a set of pages (see PDF.js for how complicated it gets).

So yeah, I was thinking that it could be nice with a new format called ".svp" for "scalable vector graphics pages" that is just this:
```xml
<svp version="0.1.0">
    <svg ...> ... </svg> <!-- page one   -->
    <svg ...> ... </svg> <!-- page two   -->
    <svg ...> ... </svg> <!-- page three -->
</svp>
```

This actually renders just fine in browsers already if you name the file .svp.html and add some styling to make it behave more like the browser pdf viewer:
```xml
<html>
<head>
<title>SVP Document</title>
<style>
body, html {
  width: 100%;
  background: #333;

  height: 100%;
}
svg {  
  display: block;
  background: white;
  box-sizing: border-box;
  padding: 2rem;
  margin: 2rem auto;
  height: 100vh;
  width: auto;
}
</style>
</head>
<body>
<svp version="0.1.0">
    <svg width="612pt" height="792pt" viewBox="0 0 612 792" version="1.1">...</svg>
    <svg width="612pt" height="792pt" viewBox="0 0 612 792" version="1.1">...</svg>
</svp>
</body>
</html>
```


It would be amazing if something like this automatically happen when a .svp file is opened (but with the same document viewer as the current browser's pdf viewer and not just this basic styling).
