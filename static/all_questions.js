$( document ).ready(function() {
  $(".unhider").click(function (e) {
    e.preventDefault();
    $(this).siblings().each(function () {
      if ($(this).hasClass("hidden")) {
        $(this).addClass("show");
        $(this).removeClass("hidden");
      } else if ($(this).hasClass("show")) {
        $(this).addClass("hidden");
        $(this).removeClass("show");
      } 
    })
  });

  $("#hint").click(function(e) {
    e.preventDefault();
    var correct_word = $("#keyword").text();
    var level = parseInt($("#hint_box").attr('level'));

    if (level == 0) {
      $("#hint_box").show();
      $("#to_put_keyword").text(correct_word);
      $("#hint").text("MORE hints");
    } else {
      var tmp = $("#correct_answer").text();
      var start_point = Math.max(tmp.indexOf(correct_word) - level * 3, 0);
      var end_point = correct_word.length + level * 6;
      var text = "<em>Part of this query:</em><br/> " + tmp.substr(start_point, end_point);
    }
    $("#hint_box").attr('level', level + 1);
    $("#hint_box").html(text);
  });

  // Test to see if this is first run and show helper screen.
  var now = new Date().getTime().toString();
  var object = JSON.parse(localStorage.getItem("has_run_before"));
  if (!object) {
    var object = {timestamp: new Date().getTime()};
    localStorage.setItem("has_run_before", JSON.stringify(object));
    $(".first_time").show();
  }

  window.onclick = function(event) {
        $(".first_time").hide();
  }


  $("#summary_box > .close").click(function(e) {
    $("#summary_box").hide();
  });

  $("#sql_to_run").focus();
});
