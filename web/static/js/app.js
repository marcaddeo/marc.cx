hljs.initHighlightingOnLoad();

jQuery.expr[':'].external = function(obj){
    return !obj.href.match(/^mailto\:/)
           && (obj.hostname != location.hostname)
           && !obj.href.match(/^javascript\:/)
           && !obj.href.match(/^$/)
};

jQuery('a:external').attr('target', '_blank');

let App = {
}
export default App
