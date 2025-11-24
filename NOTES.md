fish_add_path $HOME/.goup/bin
echo "export PATH=\"$HOME/.goup/bin:\$PATH\"" >> ~/.bashrc
echo "export PATH=\"$HOME/.goup/bin:\$PATH\"" >> ~/.zshrc

ln -s $HOME/.goup/toolchains/go1.25.4.linux-amd64/go/bin/go $HOME/.goup/bin/go

ln -s $HOME/.goup/toolchains/go1.25.4.linux-amd64/go/bin/gofmt $HOME/.goup/bin/gofmt

ln -s /usr/local/go/bin $HOME/.goup/bin

rm go gofmt

# crawling target

- https://go.dev/dl/

# Download

- https://go.dev/dl/{version}.{target}.tar.gz
- example: https://go.dev/dl/go1.24.10.linux-amd64.tar.gz
