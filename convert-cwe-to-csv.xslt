<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
    <xsl:output method="text" />
    <xsl:template match="/">
        <xsl:value-of select="//*[local-name()='Weakness_Catalog']/@Version|//*[local-name()='Weakness_Catalog']/@Catalog_Version" /><xsl:text>_</xsl:text><xsl:value-of select="//*[local-name()='Weakness_Catalog']/@Date|//*[local-name()='Weakness_Catalog']/@Catalog_Date" /><xsl:text>&#10;</xsl:text>
        <xsl:for-each select="//*[local-name()='Weakness']">
            <xsl:value-of select="@ID" /><xsl:text>&#09;</xsl:text><xsl:value-of select="@Name" /><xsl:text>&#10;</xsl:text>  
        </xsl:for-each> 
    </xsl:template>
</xsl:stylesheet>
