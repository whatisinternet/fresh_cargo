'use strict';

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
      crates: window.initial_data.crate_object,
      show_all: false
    };
  },

  componentDidMount: function () {
    setInterval(updateCrates.bind(this), 1800000)
  },

  render: function () {

    if (this.state.show_all === false) {
      var publishable = this.state.crates.filter(function(crate) {
        return crate.published === false
      })
    } else {
      var publishable = this.state.crates
    }

    return React.createElement('div', {
      className: "container"
    }, [
      React.createElement('h1', {
        key: "Title",
        className: "white-text"
      }, "Rust crates twitter bot"),
      React.createElement('a', {
        key: "all",
        className: "waves-effect waves-light btn white black-text hoverable",
        onClick: function(e) {this.setState({show_all: !this.state.show_all})}.bind(this)
      }, this.state.show_all === false ? "Show all" : "Show untweeted"),
      React.createElement('a', {
        key: "fetch",
        className: "right waves-effect waves-light btn white black-text hoverable",
        onClick: updateCrates.bind(this)
      }, "Update list"),
      React.createElement('div', {
        key: "Wrapper",
        className: 'card-panel white black-text hoverable'
      },
        React.createElement('h5', {
          key: "Title",
          className: "black-text"
        }, this.state.show_all === false ? "Showing untweeted" : "Showing all crates"),

        React.createElement('table', {
            key: "crates",
            className: 'bordered'
          },
          publishable.map(function (crate) {
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
       ),
    ])
  }
});
