
document.body.addEventListener("htmx:responseError", e => {
  let res = e.detail.xhr.responseText
  $("#htmx-error-content").html(res)
  $("#htmx-error-dialog")[0].showModal()
})

$("#htmx-error-close").click(() => {
  $("#htmx-error-dialog")[0].close()
})
