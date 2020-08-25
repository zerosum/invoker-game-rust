import React, {Component} from 'react';
import './App.css';

class App extends Component {

  constructor(props) {
    super(props);

    this.state = {
      wasm: {},
      player: {}
    };
  }

  componentDidMount() {
    this.loadWasm();
    window.addEventListener('keyup', (event) => {
      this.state.wasm.update(event.key);
      this.setState({player: this.state.wasm.fetch_status()})
    })
  }

  loadWasm = async () => {
    try {
      const wasm = await import('game');
      this.setState({wasm: wasm, player: wasm.fetch_status()});
    } catch (err) {
      console.error(`Unexpected error in loadWasm. [Message: ${err.message}]`);
    }
  };

  attachElement = (c) => {
    let base = ['element'];
    switch (c) {
      case 'Q':
        base.push('quas')
        break;
      case 'W':
        base.push('wex')
        break;
      case 'E':
        base.push('exort')
        break;
      default:
        break;
    }
    return base.join(' ')
  }

  render() {
    return (
      <div className="App">
        <header>
          <h1>Invoker Game</h1>
        </header>
        <section>

          <div>
            <span className={this.attachElement(this.state.player.e1)}/>
            <span className={this.attachElement(this.state.player.e2)}/>
            <span className={this.attachElement(this.state.player.e3)}/>
          </div>

          <table className="skills">
            <tbody>
            <tr>
              <td>[Q]</td>
              <td>Quas</td>
            </tr>
            <tr>
              <td>[W]</td>
              <td>Wex</td>
            </tr>
            <tr>
              <td>[E]</td>
              <td>Exort</td>
            </tr>
            <tr>
              <td>[R]</td>
              <td>Invoke</td>
            </tr>
            <tr>
              <td>[D]</td>
              <td>{this.state.player.s1}</td>
            </tr>
            <tr>
              <td>[F]</td>
              <td>{this.state.player.s2}</td>
            </tr>
            </tbody>
          </table>
        </section>
      </div>
    );
  }
}

export default App;
