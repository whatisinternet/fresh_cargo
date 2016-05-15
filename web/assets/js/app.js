'use strict'

var updateCrates = function() {
  $.ajax({
    url: "/feed",
    cache: false,
    success: function(data) {
      this.setState(
        {crates: JSON.parse(data).crate_object}
      );
    }.bind(this)
  })
}

window.crates = React.createClass({
  displayName: 'crates',

  getInitialState: function() {
    return {
      crates: window.initial_data.crate_object
    };
  },

  componentDidMount: function () {
    var that = this
    setInterval(updateCrates.bind(this), 1800000)
  },

  render: function () {
    return React.createElement('div', {
      className: "container"
    }, [
      React.createElement('h1', {
        key: "Title",
        className: "white-text"
      }, "Rust crates twitter bot"),
      React.createElement('div', {
        className: 'card-panel white black-text hoverable'
      },

        React.createElement('table', {
            key: "crates",
            className: 'bordered'
          },
          this.state.crates.map(function (crate) {
            return React.createElement('tbody', {
                key: crate.id
              },
              React.createElement('tr', {},
                React.createElement('td', {},
                                    React.createElement('a', {href: crate.url}, crate.name)
                                  ),
                React.createElement('td', {}, crate.version),
                React.createElement('td', {}, crate.description)
              ))
          }))
       )
    ])
  }
});
