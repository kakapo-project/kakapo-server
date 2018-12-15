cd ./dependencies/react-data-grid
yarn build
cd ../../

rm -rf ./node_modules/react-data-grid/dist
cp -r ./dependencies/react-data-grid/packages/react-data-grid/dist ./node_modules/react-data-grid/dist

