function isOdd(n)
{
    return isNumber(n) && (Math.abs(n) % 2 == 1);
}

function isNumber(n)
{
    return n == parseFloat(n);
}
(function() {
    angular.module('Utils',[])

        .factory('InfosService',[function(){
            var service = {
                message: "\n---- Logs ----\n\n",
                unplayedGrid: [],
                grid: [],
                redPlayed: [],
                bluePlayed: []
            };

            service.addMessage = function(msg){
                service.message += msg + "\n";
            };

            service.generateUnplayedGrid = function(size){
                service.unplayedGrid = [];
                for (var i = 0; i < size; i++){
                    for (var j = 0; j < size; j++){
                        service.unplayedGrid.push([i+1,j+1]);
                    }
                }
            };

            return service;
        }])

        .service('GridService',['InfosService', function(InfosService){
            var gridSize;

            this.setGridSize = function(size){
                gridSize = size;
                InfosService.generateUnplayedGrid(size);
            };

            this.getGridSize = function(){
                return gridSize;
            };

            var init = function(){
                InfosService.grid = [];
                InfosService.redPlayed = [];
                InfosService.bluePlayed = [];
                for(var i=0; i<=gridSize+1;i++){
                    InfosService.redPlayed.push([]);
                    InfosService.bluePlayed.push([]);
                }
            };

            this.makeGrid = function(){
                init();
                var totalSize = gridSize + 2;
                for (var l = 0; l < totalSize; l++){
                    InfosService.grid.push([]);
                    for(var c = 0; c < (totalSize); c++){
                        InfosService.grid[l].push([]);
                        if (l === 0 && c === 0 || l === 0 && c === totalSize - 1 ||
                            l === totalSize - 1 && c === 0 || l === totalSize - 1 && c === totalSize - 1){
                            InfosService.grid[l][c] = 'hex-default';
                        } else if (l == 0 || l == totalSize - 1){
                            InfosService.grid[l][c] = 'hex-default-red';
                        } else if (c == 0 || c == totalSize - 1){
                            InfosService.grid[l][c] = 'hex-default-blue';
                        } else {
                            InfosService.grid[l][c] = 'hex-default';
                        }
                    }
                }
            };
        }]);
})();