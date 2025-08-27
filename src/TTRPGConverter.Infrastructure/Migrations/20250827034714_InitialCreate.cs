using System;
using Microsoft.EntityFrameworkCore.Migrations;

#nullable disable

namespace TTRPGConverter.Infrastructure.Migrations
{
    /// <inheritdoc />
    public partial class InitialCreate : Migration
    {
        /// <inheritdoc />
        protected override void Up(MigrationBuilder migrationBuilder)
        {
            migrationBuilder.CreateTable(
                name: "CompendiumItems",
                columns: table => new
                {
                    Id = table.Column<string>(type: "TEXT", nullable: false),
                    Name = table.Column<string>(type: "TEXT", nullable: false),
                    Type = table.Column<string>(type: "TEXT", nullable: false),
                    Data = table.Column<string>(type: "TEXT", nullable: false),
                    SourceFormat = table.Column<string>(type: "TEXT", nullable: false),
                    SourceFile = table.Column<string>(type: "TEXT", nullable: false),
                    Description = table.Column<string>(type: "TEXT", nullable: true),
                    Source = table.Column<string>(type: "TEXT", nullable: true),
                    Rarity = table.Column<string>(type: "TEXT", nullable: true),
                    LoadedAt = table.Column<DateTime>(type: "TEXT", nullable: false),
                    IsPrimary = table.Column<bool>(type: "INTEGER", nullable: false),
                    System = table.Column<string>(type: "TEXT", nullable: true)
                },
                constraints: table =>
                {
                    table.PrimaryKey("PK_CompendiumItems", x => x.Id);
                });

            migrationBuilder.CreateIndex(
                name: "IX_CompendiumItems_Name_Type_System",
                table: "CompendiumItems",
                columns: new[] { "Name", "Type", "System" });
        }

        /// <inheritdoc />
        protected override void Down(MigrationBuilder migrationBuilder)
        {
            migrationBuilder.DropTable(
                name: "CompendiumItems");
        }
    }
}
