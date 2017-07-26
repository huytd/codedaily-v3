const path = require('path');

module.exports = {
  entry: path.join(__dirname, 'main.js'),
  output: {
    filename: 'build.js',
    path: __dirname
  },
  devServer: {
    contentBase: __dirname,
    inline: true
  },
  resolve: {
    extensions: ['.js', '.jsx']
  },
  module: {
    loaders: [
      {
        test: /\.jsx?/,
        exclude: /(node_modules)/,
        loader: 'babel-loader',
        options: {
          presets: ['es2015', 'react']
        }
      },
      {
        test: /\.scss$/,
        loaders: ['style-loader', 'css-loader', 'sass-loader']
      }
    ]
  }
};
