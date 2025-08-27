using Microsoft.EntityFrameworkCore.Migrations;

#nullable disable

namespace TTRPGConverter.Infrastructure.Migrations
{
    /// <inheritdoc />
    public partial class AddAssetMappings : Migration
    {
        /// <inheritdoc />
        protected override void Up(MigrationBuilder migrationBuilder)
        {
            migrationBuilder.CreateTable(
                name: "AssetMappings",
                columns: table => new
                {
                    SourceUrl = table.Column<string>(type: "TEXT", nullable: false),
                    LocalPath = table.Column<string>(type: "TEXT", nullable: false)
                },
                constraints: table =>
                {
                    table.PrimaryKey("PK_AssetMappings", x => x.SourceUrl);
                });
        }

        /// <inheritdoc />
        protected override void Down(MigrationBuilder migrationBuilder)
        {
            migrationBuilder.DropTable(
                name: "AssetMappings");
        }
    }
}
